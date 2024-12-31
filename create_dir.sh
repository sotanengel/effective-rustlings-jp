#!/bin/bash
# 参照元：https://www.lurklurk.org/effective-rust/title-page.html

BASE_DIRS=("exercises" "solutions")

FOLDERS=(
  "1_types_Use_the_type_system_to_express_your_data_structures"
  "2_types_Use_the_type_system_to_express_common_behavior"
  "3_types_Prefer_option_and_result_transforms_over_explicit_match_expressions"
  "4_types_Prefer_idiomatic_error_types"
  "5_types_Understand_type_conversions"
  "6_types_Embrace_the_newtype_pattern"
  "7_types_Use_builders_for_complex_types"
  "8_types_Familiarize_yourself_with_reference_and_pointer_types"
  "9_types_Consider_using_iterator_transforms_instead_of_explicit_loops"
  "10_traits_Familiarize_yourself_with_standard_traits"
  "11_traits_Implement_the_drop_trait_for_RAII_patterns"
  "12_traits_Understand_the_trade_offs_between_generics_and_trait_objects"
  "13_traits_Use_default_implementations_to_minimize_required_trait_methods"
  "14_concepts_Understand_lifetimes"
  "15_concepts_Understand_the_borrow_checker"
  "16_concepts_Avoid_writing_unsafe_code"
  "17_concepts_Be_wary_of_shared_state_parallelism"
  "18_concepts_Dont_panic"
  "19_concepts_Avoid_reflection"
  "20_concepts_Avoid_the_temptation_to_over_optimize"
  "21_dependencies_Understand_what_semantic_versioning_promises"
  "22_dependencies_Minimize_visibility"
  "23_dependencies_Avoid_wildcard_imports"
  "24_dependencies_Re_export_dependencies_whose_types_appear_in_your_api"
  "25_dependencies_Manage_your_dependency_graph"
  "26_dependencies_Be_wary_of_feature_creep"
  "27_tooling_Document_public_interfaces"
  "28_tooling_Use_macros_judiciously"
  "29_tooling_Listen_to_clippy"
  "30_tooling_Write_more_than_unit_tests"
  "31_tooling_Take_advantage_of_the_tooling_ecosystem"
  "32_tooling_Set_up_a_continuous_integration_ci_system"
  "33_beyond_standard_rust_Consider_making_library_code_no_std_compatible"
  "34_beyond_standard_rust_Control_what_crosses_ffi_boundaries"
  "35_beyond_standard_rust_Prefer_bindgen_to_manual_ffi_mappings"
)

for BASE in "${BASE_DIRS[@]}"; do
  for FOLDER in "${FOLDERS[@]}"; do
    mkdir -p "$BASE/$FOLDER"
  done
done
