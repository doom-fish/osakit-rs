# osakit-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 91
VERIFIED: 89
GAPS: 0
EXEMPT: 2
COVERAGE_PCT: 100.0

Audit performed by enumerating all public Objective-C interfaces, methods, properties, and constants from the five OSAKit framework headers (OSALanguage.h, OSALanguageInstance.h, OSAScript.h, OSAScriptView.h, OSAScriptController.h) in MacOSX26.2.sdk. The crate exposes these symbols via a Rust safe wrapper backed by a Swift bridge layer. All 91 public macOS-available symbols are accounted for: 89 are actively wrapped and 2 deprecated constructors (10.6+) are intentionally omitted in favor of non-deprecated alternatives.

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `OSALanguage` | class | `OSALanguage.h` | `Language` |
| `OSALanguageFeatures` | option set | `OSALanguage.h` | `LanguageFeatures`, `LanguageSummary::feature_flags()` |
| `+availableLanguages` | class method | `OSALanguage.h` | `Language::available_languages()` |
| `+languageForName:` | class method | `OSALanguage.h` | `Language::for_name()` |
| `+languageForScriptDataDescriptor:` | class method | `OSALanguage.h` | `Language::for_script_data_descriptor()` |
| `+defaultLanguage` | class method | `OSALanguage.h` | `Language::default_language()` |
| `+setDefaultLanguage:` | class method | `OSALanguage.h` | `Language::set_default()`, `Language::set_as_default()` |
| `-initWithComponent:` | initializer | `OSALanguage.h` | `OsaComponent::from_language()` round-trip, paired with `Language::component()` |
| `-sharedLanguageInstance` | instance method | `OSALanguage.h` | `Language::shared_instance()` |
| `componentInstance` | property | `OSALanguage.h` | `OsaComponentInstance::from_language()` |
| `name`, `info`, `version`, `type`, `subType`, `manufacturer`, `features`, `threadSafe` | properties (8) | `OSALanguage.h` | `Language::name()`, `Language::summary()`, `LanguageSummary`, `LanguageFeatures` |
| `OSALanguageInstance` | class | `OSALanguageInstance.h` | `LanguageInstance` |
| `+languageInstanceWithLanguage:`, `-initWithLanguage:` | constructor family | `OSALanguageInstance.h` | `LanguageInstance::new()` |
| `language` | property | `OSALanguageInstance.h` | `LanguageInstance::language()` |
| `componentInstance` | property | `OSALanguageInstance.h` | `LanguageInstance::component_instance()`, `OsaComponentInstance::from_language_instance()` |
| `defaultTarget` | property | `OSALanguageInstance.h` | `LanguageInstance::default_target()`, `LanguageInstance::set_default_target()` |
| `-richTextFromDescriptor:` | instance method | `OSALanguageInstance.h` | `LanguageInstance::rich_text_from_descriptor()` |
| `OSAScript` | class | `OSAScript.h` | `Script` |
| `OSAScriptErrorMessageKey`, `OSAScriptErrorBriefMessageKey`, `OSAScriptErrorNumberKey`, `OSAScriptErrorPartialResultKey`, `OSAScriptErrorOffendingObjectKey`, `OSAScriptErrorExpectedTypeKey`, `OSAScriptErrorAppAddressKey`, `OSAScriptErrorAppNameKey`, `OSAScriptErrorRangeKey` | constants (9) | `OSAScript.h` | `script_error_constants()`, `ScriptErrorConstants`, `ScriptErrorDetails` |
| `OSAScriptErrorMessage`, `OSAScriptErrorNumber`, `OSAScriptErrorAppName`, `OSAScriptErrorBriefMessage`, `OSAScriptErrorRange` | legacy constants (5) | `OSAScript.h` | `script_error_constants()`, `ScriptErrorConstants`, `ScriptErrorDetails` |
| `OSAStorageScriptType`, `OSAStorageScriptBundleType`, `OSAStorageApplicationType`, `OSAStorageApplicationBundleType`, `OSAStorageTextType` | constants (5) | `OSAScript.h` | `ScriptStorageType::{Script, ScriptBundle, Application, ApplicationBundle, Text}` |
| `OSAStorageOptions` | option set | `OSAScript.h` | `StorageOptions` |
| `+scriptDataDescriptorWithContentsOfURL:` | class method | `OSAScript.h` | `Script::script_data_descriptor_from_file()` |
| `-initWithSource:` | initializer | `OSAScript.h` | `Script::new(source, None)` |
| `-initWithSource:language:` | initializer | `OSAScript.h` | `Script::new(source, Some(language))` |
| `-initWithSource:fromURL:languageInstance:usingStorageOptions:` | initializer | `OSAScript.h` | `Script::from_source_with_options()` |
| `-initWithContentsOfURL:error:` | initializer | `OSAScript.h` | `Script::from_file(path, None)` |
| `-initWithContentsOfURL:languageInstance:usingStorageOptions:error:` | initializer | `OSAScript.h` | `Script::from_file_with_options()`; `Script::from_file(path, Some(language))` routes through the same non-deprecated language-instance path |
| `-initWithCompiledData:fromURL:usingStorageOptions:error:` | initializer | `OSAScript.h` | `Script::from_compiled_data()` |
| `-initWithScriptDataDescriptor:fromURL:languageInstance:usingStorageOptions:error:` | initializer | `OSAScript.h` | `Script::from_script_data_descriptor()` |
| `source` | property | `OSAScript.h` | `Script::source()` |
| `url` | property | `OSAScript.h` | `Script::url()` |
| `language` | property | `OSAScript.h` | `Script::language()`, `Script::set_language()` |
| `languageInstance` | property | `OSAScript.h` | `Script::language_instance()`, `Script::set_language_instance()` |
| `compiled` | property | `OSAScript.h` | `Script::is_compiled()` |
| `-compileAndReturnError:` | instance method | `OSAScript.h` | `Script::compile()` |
| `-executeAndReturnError:` | instance method | `OSAScript.h` | `Script::execute()` |
| `-executeAppleEvent:error:` | instance method | `OSAScript.h` | `Script::execute_apple_event()` |
| `-executeAndReturnDisplayValue:error:` | instance method | `OSAScript.h` | `Script::execute_and_return_display_value()` |
| `-executeHandlerWithName:arguments:error:` | instance method | `OSAScript.h` | `Script::execute_handler()` |
| `richTextSource` | property | `OSAScript.h` | `Script::rich_text_source()` |
| `-richTextFromDescriptor:` | instance method | `OSAScript.h` | `Script::rich_text_from_descriptor()` |
| `-writeToURL:ofType:error:` | instance method | `OSAScript.h` | `Script::write_to_file(..., StorageOptions::NONE)` |
| `-writeToURL:ofType:usingStorageOptions:error:` | instance method | `OSAScript.h` | `Script::write_to_file(...)` |
| `-compiledDataForType:usingStorageOptions:error:` | instance method | `OSAScript.h` | `Script::compiled_data()` |
| `OSAScriptView` | class | `OSAScriptView.h` | `ScriptView` |
| `source` | property | `OSAScriptView.h` | `ScriptView::source()`, `ScriptView::set_source()` |
| `usesScriptAssistant` | property | `OSAScriptView.h` | `ScriptView::uses_script_assistant()`, `ScriptView::set_uses_script_assistant()` |
| `usesTabs` | property | `OSAScriptView.h` | `ScriptView::uses_tabs()`, `ScriptView::set_uses_tabs()` |
| `tabWidth` | property | `OSAScriptView.h` | `ScriptView::tab_width()`, `ScriptView::set_tab_width()` |
| `wrapsLines` | property | `OSAScriptView.h` | `ScriptView::wraps_lines()`, `ScriptView::set_wraps_lines()` |
| `indentsWrappedLines` | property | `OSAScriptView.h` | `ScriptView::indents_wrapped_lines()`, `ScriptView::set_indents_wrapped_lines()` |
| `indentWidth` | property | `OSAScriptView.h` | `ScriptView::indent_width()`, `ScriptView::set_indent_width()` |
| `OSAScriptController` | class | `OSAScriptController.h` | `ScriptController` |
| `OSAScriptState` | enum | `OSAScriptController.h` | `ScriptState` |
| `scriptView` | property | `OSAScriptController.h` | `ScriptController::script_view()`, `ScriptController::set_script_view()` |
| `resultView` | property | `OSAScriptController.h` | `ScriptController::result_text()` (safe projection of the backing `NSTextView`) |
| `script` | property | `OSAScriptController.h` | `ScriptController::script()`, `ScriptController::set_script()` |
| `language` | property | `OSAScriptController.h` | `ScriptController::language()`, `ScriptController::set_language()` |
| `scriptState` | property | `OSAScriptController.h` | `ScriptController::script_state()`, `ScriptState` |
| `compiling` | property | `OSAScriptController.h` | `ScriptController::is_compiling()` |
| `-compileScript:` | action | `OSAScriptController.h` | `ScriptController::compile_script()` |
| `-recordScript:` | action | `OSAScriptController.h` | `ScriptController::record_script()` |
| `-runScript:` | action | `OSAScriptController.h` | `ScriptController::run_script()` |
| `-stopScript:` | action | `OSAScriptController.h` | `ScriptController::stop_script()` |

## 🔴 GAPS

| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| None | - | - | No uncovered public macOS OSAKit surface found after excluding the two deprecated constructors below. |

## ⏭️ EXEMPT

| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `-initWithContentsOfURL:language:error:` | deprecated initializer | `OSAScript.h` | 10.6-deprecated constructor; crate routes language-based file loading through `sharedLanguageInstance()` and the non-deprecated `Script::from_file_with_options()` path instead. | `DEPRECATED_IN_MAC_OS_X_VERSION_10_6_AND_LATER` |
| `-initWithCompiledData:error:` | deprecated initializer | `OSAScript.h` | 10.6-deprecated constructor; crate intentionally exposes `Script::from_compiled_data()` backed by the non-deprecated URL/storage-options initializer. | `DEPRECATED_IN_MAC_OS_X_VERSION_10_6_AND_LATER` |
