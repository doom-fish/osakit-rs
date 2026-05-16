# OSAKit.framework coverage audit

Audit source: `OSAKit.framework/Headers/*.h` from the active macOS SDK (`xcrun --sdk macosx --show-sdk-path`).

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped — deprecated / legacy alias / platform constraint

## OSAKit.h

| API | Status | Notes |
| --- | --- | --- |
| Umbrella import for `OSALanguage`, `OSALanguageInstance`, `OSAScript`, `OSAScriptView`, `OSAScriptController` | ✅ | Covered by the area modules below. |

## OSALanguage.h

| API | Status | Notes |
| --- | --- | --- |
| `OSALanguageFeatures` | ✅ | Exposed as `LanguageFeatures` bitflags plus `LanguageSummary::feature_flags()`. |
| `+availableLanguages` | ✅ | `Language::available_languages()`. |
| `+languageForName:` | ✅ | `Language::for_name()`. |
| `+languageForScriptDataDescriptor:` | ✅ | `Language::for_script_data_descriptor()`. |
| `+defaultLanguage` | ✅ | `Language::default_language()`. |
| `+setDefaultLanguage:` | ✅ | `Language::set_default()` / `set_as_default()`. |
| `-initWithComponent:` | ✅ | Represented by `OsaComponent::language()` and `Language::component()` round-trips. |
| `-sharedLanguageInstance` | ✅ | `Language::shared_instance()`. |
| `componentInstance` | ✅ | `OsaComponentInstance::from_language()` / `Language::component()`. |
| `name` / `info` / `version` / `type` / `subType` / `manufacturer` / `features` / `threadSafe` | ✅ | Available through `Language::summary()` and `LanguageSummary`. |

## OSALanguageInstance.h

| API | Status | Notes |
| --- | --- | --- |
| `+languageInstanceWithLanguage:` | ✅ | Represented by `LanguageInstance::new()`. |
| `-initWithLanguage:` | ✅ | `LanguageInstance::new()`. |
| `language` | ✅ | `LanguageInstance::language()`. |
| `componentInstance` | ✅ | `LanguageInstance::component_instance()` / `OsaComponentInstance`. |
| `defaultTarget` | ✅ | `LanguageInstance::default_target()` / `set_default_target()`. |
| `-richTextFromDescriptor:` | ✅ | `LanguageInstance::rich_text_from_descriptor()`. |

## OSAScript.h

| API | Status | Notes |
| --- | --- | --- |
| `OSAScriptErrorMessageKey` / `BriefMessageKey` / `NumberKey` / `PartialResultKey` / `OffendingObjectKey` / `ExpectedTypeKey` / `AppAddressKey` / `AppNameKey` / `RangeKey` | ✅ | `script_error_constants()` and `ScriptErrorConstants`. |
| `OSAScriptErrorMessage` / `Number` / `AppName` / `BriefMessage` / `Range` legacy aliases | ✅ | Included in `ScriptErrorConstants`. |
| `OSAStorageScriptType` / `ScriptBundleType` / `ApplicationType` / `ApplicationBundleType` / `TextType` | ✅ | `ScriptStorageType`. |
| `OSAStorageOptions` | ✅ | `StorageOptions` bitflags. |
| `+scriptDataDescriptorWithContentsOfURL:` | ✅ | `Script::script_data_descriptor_from_file()`. |
| `-initWithSource:` | ✅ | `Script::new()` with `None` language. |
| `-initWithSource:language:` | ✅ | `Script::new()` with an explicit `Language`. |
| `-initWithSource:fromURL:languageInstance:usingStorageOptions:` | ✅ | `Script::from_source_with_options()`. |
| `-initWithContentsOfURL:error:` | ✅ | `Script::from_file()` without a language. |
| `-initWithContentsOfURL:language:error:` | ⏭️ skipped | Deprecated 10.5-era constructor superseded by language-instance loading. |
| `-initWithContentsOfURL:languageInstance:usingStorageOptions:error:` | ✅ | `Script::from_file_with_options()`. |
| `-initWithCompiledData:error:` | ⏭️ skipped | Deprecated 10.5-era constructor superseded by URL/storage-options loader. |
| `-initWithCompiledData:fromURL:usingStorageOptions:error:` | ✅ | `Script::from_compiled_data()`. |
| `-initWithScriptDataDescriptor:fromURL:languageInstance:usingStorageOptions:error:` | ✅ | `Script::from_script_data_descriptor()`. |
| `source` | ✅ | `Script::source()`. |
| `url` | ✅ | `Script::url()`. |
| `language` | ✅ | `Script::language()` / `set_language()`. |
| `languageInstance` | ✅ | `Script::language_instance()` / `set_language_instance()`. |
| `compiled` | ✅ | `Script::is_compiled()`. |
| `-compileAndReturnError:` | ✅ | `Script::compile()`. |
| `-executeAndReturnError:` | ✅ | `Script::execute()`. |
| `-executeAppleEvent:error:` | ✅ | `Script::execute_apple_event()`. |
| `-executeAndReturnDisplayValue:error:` | ✅ | `Script::execute_and_return_display_value()`. |
| `-executeHandlerWithName:arguments:error:` | ✅ | `Script::execute_handler()`. |
| `richTextSource` | ✅ | `Script::rich_text_source()`. |
| `-richTextFromDescriptor:` | ✅ | `Script::rich_text_from_descriptor()`. |
| `-writeToURL:ofType:error:` | ✅ | `Script::write_to_file()` with `StorageOptions::NONE`. |
| `-writeToURL:ofType:usingStorageOptions:error:` | ✅ | `Script::write_to_file()` with explicit options. |
| `-compiledDataForType:usingStorageOptions:error:` | ✅ | `Script::compiled_data()`. |

## OSAScriptController.h

| API | Status | Notes |
| --- | --- | --- |
| `OSAScriptState` | ✅ | `ScriptState`. |
| `scriptView` | ✅ | `ScriptController::script_view()` / `set_script_view()`. |
| `resultView` | ✅ | Exposed safely through `ScriptController::result_text()` while the bridge manages the backing `NSTextView`. |
| `script` | ✅ | `ScriptController::script()` / `set_script()`. |
| `language` | ✅ | `ScriptController::language()` / `set_language()`. |
| `scriptState` | ✅ | `ScriptController::script_state()`. |
| `compiling` | ✅ | `ScriptController::is_compiling()`. |
| `-compileScript:` | ✅ | `ScriptController::compile_script()`. |
| `-recordScript:` | ✅ | `ScriptController::record_script()`. |
| `-runScript:` | ✅ | `ScriptController::run_script()`. |
| `-stopScript:` | ✅ | `ScriptController::stop_script()`. |

## OSAScriptView.h

| API | Status | Notes |
| --- | --- | --- |
| `source` | ✅ | `ScriptView::source()` / `set_source()`. |
| `usesScriptAssistant` | ✅ | `ScriptView::uses_script_assistant()` / `set_uses_script_assistant()`. |
| `usesTabs` | ✅ | `ScriptView::uses_tabs()` / `set_uses_tabs()`. |
| `tabWidth` | ✅ | `ScriptView::tab_width()` / `set_tab_width()`. |
| `wrapsLines` | ✅ | `ScriptView::wraps_lines()` / `set_wraps_lines()`. |
| `indentsWrappedLines` | ✅ | `ScriptView::indents_wrapped_lines()` / `set_indents_wrapped_lines()`. |
| `indentWidth` | ✅ | `ScriptView::indent_width()` / `set_indent_width()`. |
