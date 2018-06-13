//! The MDCCC CLI

#![doc(html_root_url = "https://docs.rs/mdccc/0.1.0")]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications, missing_docs)]
#![cfg_attr(feature = "cargo-clippy",
            allow(single_match, match_same_arms, match_ref_pats,
                  clone_on_ref_ptr, needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy",
            deny(wrong_pub_self_convention, used_underscore_binding,
                 stutter, similar_names, pub_enum_variant_names,
                 missing_docs_in_private_items,
                 non_ascii_literal, unicode_not_nfc,
                 result_unwrap_used, option_unwrap_used,
                 option_map_unwrap_or_else, option_map_unwrap_or, filter_map,
                 shadow_unrelated, shadow_reuse, shadow_same,
                 int_plus_one, string_add_assign, if_not_else,
                 invalid_upcast_comparisons,
                 cast_precision_loss, cast_lossless,
                 cast_possible_wrap, cast_possible_truncation,
                 mutex_integer, mut_mut, items_after_statements,
                 mem_forget, maybe_infinite_iter))]

extern crate mdccc;

use std::io::{ stdin, stdout };
use std::io::Read;
use mdccc::latex::LaTeXIter;

fn main() {
    let input = {
        let mut s = String::new();
        stdin().read_to_string(&mut s).map(drop).unwrap_or_else(
            |e| eprintln!("can't read from stdin: {}", e)
        );
        s
    };
    let mut iter = LaTeXIter::with_str(&input);

    iter.write_to_io(&mut stdout()).unwrap_or_else(
        |e| eprintln!("can't write LaTeX: {}", e)
    );
}
