#!/bin/bash
# 参照元：https://www.lurklurk.org/effective-rust/title-page.html

BASE_DIRS=("exercises" "solutions")

FOLDERS=(
  "1-types/1-use-the-type-system-to-express-your-data-structures"
  "1-types/2-use-the-type-system-to-express-common-behavior"
  "1-types/3-prefer-option-and-result-transforms-over-explicit-match-expressions"
  "1-types/4-prefer-idiomatic-error-types"
  "1-types/5-understand-type-conversions"
  "1-types/6-embrace-the-newtype-pattern"
  "1-types/7-use-builders-for-complex-types"
  "1-types/8-familiarize-yourself-with-reference-and-pointer-types"
  "1-types/9-consider-using-iterator-transforms-instead-of-explicit-loops"
  "2-traits/1-familiarize-yourself-with-standard-traits"
  "2-traits/2-implement-the-drop-trait-for-RAII-patterns"
  "2-traits/3-understand-the-trade-offs-between-generics-and-trait-objects"
  "2-traits/4-use-default-implementations-to-minimize-required-trait-methods"
  "3-concepts/1-understand-lifetimes"
  "3-concepts/2-understand-the-borrow-checker"
  "3-concepts/3-avoid-writing-unsafe-code"
  "3-concepts/4-be-wary-of-shared-state-parallelism"
  "3-concepts/5-dont-panic"
  "3-concepts/6-avoid-reflection"
  "3-concepts/7-avoid-the-temptation-to-over-optimize"
  "4-dependencies/1-understand-what-semantic-versioning-promises"
  "4-dependencies/2-minimize-visibility"
  "4-dependencies/3-avoid-wildcard-imports"
  "4-dependencies/4-re-export-dependencies-whose-types-appear-in-your-api"
  "4-dependencies/5-manage-your-dependency-graph"
  "4-dependencies/6-be-wary-of-feature-creep"
  "5-tooling/1-document-public-interfaces"
  "5-tooling/2-use-macros-judiciously"
  "5-tooling/3-listen-to-clippy"
  "5-tooling/4-write-more-than-unit-tests"
  "5-tooling/5-take-advantage-of-the-tooling-ecosystem"
  "5-tooling/6-set-up-a-continuous-integration-ci-system"
  "6-beyond-standard-rust/1-consider-making-library-code-no-std-compatible"
  "6-beyond-standard-rust/2-control-what-crosses-ffi-boundaries"
  "6-beyond-standard-rust/3-prefer-bindgen-to-manual-ffi-mappings"
)

for BASE in "${BASE_DIRS[@]}"; do
  for FOLDER in "${FOLDERS[@]}"; do
    mkdir -p "$BASE/$FOLDER"
  done
done
