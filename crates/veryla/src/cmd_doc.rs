use crate::doc::{DocBuilder, TopLevelItem};
use crate::OptDoc;
use log::info;
use miette::{IntoDiagnostic, Result, WrapErr};
use std::collections::BTreeMap;
use std::fs;
use veryla_analyzer::symbol::{SymbolId, SymbolKind};
use veryla_analyzer::{symbol_table, Analyzer};
use veryla_metadata::Metadata;
use veryla_parser::resource_table;
use veryla_parser::Parser;

pub struct CmdDoc {
    opt: OptDoc,
}

impl CmdDoc {
    pub fn new(opt: OptDoc) -> Self {
        Self { opt }
    }

    pub fn exec(&self, metadata: &mut Metadata) -> Result<bool> {
        let paths = metadata.paths(&self.opt.files, true)?;

        let mut contexts = Vec::new();

        for path in &paths {
            info!("Processing file ({})", path.src.to_string_lossy());

            let input = fs::read_to_string(&path.src)
                .into_diagnostic()
                .wrap_err("")?;
            let parser = Parser::parse(&input, &path.src)?;
            let analyzer = Analyzer::new(metadata);
            analyzer.analyze_pass1(&path.prj, &input, &path.src, &parser.veryla);

            contexts.push((path, input, parser, analyzer));
        }

        Analyzer::analyze_post_pass1();

        for (path, input, parser, analyzer) in &contexts {
            analyzer.analyze_pass2(&path.prj, input, &path.src, &parser.veryla);
        }

        for (path, input, parser, analyzer) in &contexts {
            analyzer.analyze_pass3(&path.prj, input, &path.src, &parser.veryla);
        }

        let mut entitys = BTreeMap::new();
        let mut proto_entitys = BTreeMap::new();
        let mut interfaces = BTreeMap::new();
        let mut packages = BTreeMap::new();

        for symbol in veryla_analyzer::symbol_table::get_all() {
            let text = resource_table::get_str_value(symbol.token.text).unwrap();
            let file_name = text.clone();
            let symbol = symbol.clone();
            if format!("{}", symbol.namespace) == metadata.project.name && symbol.public {
                match &symbol.kind {
                    SymbolKind::Entity(x) => {
                        let html_name = fmt_generic_parameters(&text, &x.generic_parameters);
                        let item = TopLevelItem {
                            file_name,
                            html_name,
                            symbol,
                        };
                        entitys.insert(text, item);
                    }
                    SymbolKind::ProtoEntity(_) => {
                        let html_name = file_name.clone();
                        let item = TopLevelItem {
                            file_name,
                            html_name,
                            symbol,
                        };
                        proto_entitys.insert(text, item);
                    }
                    SymbolKind::Interface(x) => {
                        let html_name = fmt_generic_parameters(&text, &x.generic_parameters);
                        let item = TopLevelItem {
                            file_name,
                            html_name,
                            symbol,
                        };
                        interfaces.insert(text, item);
                    }
                    SymbolKind::Package(x) => {
                        let html_name = fmt_generic_parameters(&text, &x.generic_parameters);
                        let item = TopLevelItem {
                            file_name,
                            html_name,
                            symbol,
                        };
                        packages.insert(text, item);
                    }
                    _ => (),
                }
            }
        }

        let entitys: Vec<_> = entitys.into_values().collect();
        let proto_entitys: Vec<_> = proto_entitys.into_values().collect();
        let interfaces: Vec<_> = interfaces.into_values().collect();
        let packages: Vec<_> = packages.into_values().collect();

        let builder = DocBuilder::new(metadata, entitys, proto_entitys, interfaces, packages)?;
        builder.build()?;

        Ok(true)
    }
}

fn fmt_generic_parameters(name: &str, params: &[SymbolId]) -> String {
    if params.is_empty() {
        name.to_string()
    } else {
        let mut name = name.to_string();
        name.push_str("::&lt;");
        for param in params {
            let symbol = symbol_table::get(*param).unwrap();
            name.push_str(&format!("{}, ", symbol.token.text));
        }
        format!("{}&gt;", name.strip_suffix(", ").unwrap())
    }
}
