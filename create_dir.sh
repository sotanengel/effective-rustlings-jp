#!/bin/bash
# 参照元：https://www.lurklurk.org/effective-rust/title-page.html

BASE_DIRS=("exercises" "solutions")

FOLDERS=(
  "1_types/1_use_the_type_system_to_express_your_data_structures"
  "1_types/2_use_the_type_system_to_express_common_behavior"
  "1_types/3_prefer_option_and_result_transforms_over_explicit_match_expressions"
  "1_types/4_prefer_idiomatic_error_types"
  "1_types/5_understand_type_conversions"
  "1_types/6_embrace_the_newtype_pattern"
  "1_types/7_use_builders_for_complex_types"
  "1_types/8_familiarize_yourself_with_reference_and_pointer_types"
  "1_types/9_consider_using_iterator_transforms_instead_of_explicit_loops"
  "2_traits/1_familiarize_yourself_with_standard_traits"
  "2_traits/2_implement_the_drop_trait_for_RAII_patterns"
  "2_traits/3_understand_the_trade_offs_between_generics_and_trait_objects"
  "2_traits/4_use_default_implementations_to_minimize_required_trait_methods"
  "3_concepts/1_understand_lifetimes"
  "3_concepts/2_understand_the_borrow_checker"
  "3_concepts/3_avoid_writing_unsafe_code"
  "3_concepts/4_be_wary_of_shared_state_parallelism"
  "3_concepts/5_dont_panic"
  "3_concepts/6_avoid_reflection"
  "3_concepts/7_avoid_the_temptation_to_over_optimize"
  "4_dependencies/1_understand_what_semantic_versioning_promises"
  "4_dependencies/2_minimize_visibility"
  "4_dependencies/3_avoid_wildcard_imports"
  "4_dependencies/4_re_export_dependencies_whose_types_appear_in_your_api"
  "4_dependencies/5_manage_your_dependency_graph"
  "4_dependencies/6_be_wary_of_feature_creep"
  "5_tooling/1_document_public_interfaces"
  "5_tooling/2_use_macros_judiciously"
  "5_tooling/3_listen_to_clippy"
  "5_tooling/4_write_more_than_unit_tests"
  "5_tooling/5_take_advantage_of_the_tooling_ecosystem"
  "5_tooling/6_set_up_a_continuous_integration_ci_system"
  "6_beyond_standard_rust/1_consider_making_library_code_no_std_compatible"
  "6_beyond_standard_rust/2_control_what_crosses_ffi_boundaries"
  "6_beyond_standard_rust/3_prefer_bindgen_to_manual_ffi_mappings"
)

for BASE in "${BASE_DIRS[@]}"; do
  for FOLDER in "${FOLDERS[@]}"; do
    mkdir -p "$BASE/$FOLDER"
  done
done
