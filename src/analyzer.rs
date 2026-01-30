use crate::config::{ArchError, LinterConfig};
use miette::{IntoDiagnostic, Result, SourceSpan};
use std::path::PathBuf;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

pub fn analyze_file(cm: &SourceMap, path: &PathBuf, config: &LinterConfig) -> Result<()> {
    let fm = cm.load_file(path).into_diagnostic()?;
    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            decorators: true,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| miette::miette!("{:?}", e))?;

    let file_name = path.to_string_lossy();

    for item in &module.body {
        // REGLA 1: Imports en Controllers
        if file_name.ends_with(".controller.ts") {
            if let swc_ecma_ast::ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(import)) =
                item
            {
                if import.src.value.contains(".repository") {
                    let start = (import.span.lo.0 - fm.start_pos.0) as usize;
                    let end = (import.span.hi.0 - fm.start_pos.0) as usize;
                    return Err(ArchError {
                        src: fm.src.to_string(),
                        span: SourceSpan::new(start.into(), (end - start).into()),
                        message: "Prohibido importar repositorios en controladores. Usa servicios."
                            .into(),
                    }
                    .into());
                }
            }
        }

        // REGLA 2: Métodos de Clase (LOC)
        if let swc_ecma_ast::ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(
            swc_ecma_ast::Decl::Class(c),
        )) = item
        {
            for member in &c.class.body {
                if let swc_ecma_ast::ClassMember::Method(m) = member {
                    let lo = cm.lookup_char_pos(m.span.lo).line;
                    let hi = cm.lookup_char_pos(m.span.hi).line;
                    let lines = hi - lo;

                    if lines > config.max_lines_per_function {
                        let start = (m.span.lo.0 - fm.start_pos.0) as usize;
                        let end = (m.span.hi.0 - fm.start_pos.0) as usize;
                        return Err(ArchError {
                            src: fm.src.to_string(),
                            span: SourceSpan::new(start.into(), (end - start).into()),
                            message: format!(
                                "Método demasiado largo ({} líneas). Máximo: {}",
                                lines, config.max_lines_per_function
                            ),
                        }
                        .into());
                    }
                }
            }
        }
    }
    Ok(())
}
