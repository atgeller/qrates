use crate::write_csv;
use corpus_database::tables::Loader;
use std::collections::HashMap;
use std::path::Path;

/// Report information about all calls in our codebase.
fn report_all_calls(loader: &Loader, report_path: &Path) {
    let def_paths = loader.load_def_paths();

    //let selected_builds = loader.load_selected_builds();
    let terminators_call_const_target = loader.load_terminators_call_const_target();
    let terminators_call_const_target: HashMap<_, _> =
        terminators_call_const_target.iter().copied().collect();
    let strings = loader.load_strings();
    let summary_keys = loader.load_summary_keys();
    
    let basic_blocks = loader.load_basic_blocks();
    let basic_blocks: HashMap<_,_> = basic_blocks.iter().map(|&(block, def_path, _expansion_kind)| (block, def_path)).collect();
    let crate_names = loader.load_crate_names();

    let all_calls = loader.load_terminators_call();
    let slice_get_unchecked_calls = all_calls.iter().map(
        |&(block, call, _, _, _, _, _, _, _)| {
            if let Some(target) = terminators_call_const_target.get(&call)
            {
                let (_, _, _, _, summary_key) = def_paths[*target];
                let call_target = strings[summary_keys[summary_key]].to_string();
                if let Some(_) = call_target.find("core.slice.implement.get_unchecked") {
                    let index = call_target.find("get_unchecked").unwrap();
                    let (_, func_name) = call_target.split_at(index);
                    let (crate_name, _, _, _, _) = def_paths[basic_blocks[&block]];

                    return (
                        strings[crate_names[crate_name]].as_ref(),
                        func_name.into(),
                    );
                }
            }
            ("","".into()) 
        },
    ).filter(|(_, func_name): &(&str, String)| func_name.len() > 0);
    write_csv!(report_path, slice_get_unchecked_calls);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_all_calls(loader, report_path);
}