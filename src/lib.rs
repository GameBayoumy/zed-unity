use zed_extension_api::{
    self as zed,
    lsp::{Completion, Symbol},
};

struct MyExtension {
    // ... state
}

impl zed::Extension for MyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Err("`language_server_command` not implemented".to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_additional_initialization_options(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _target_language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _target_language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        Ok(None)
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _completion: Completion,
    ) -> Option<zed_extension_api::CodeLabel> {
        None
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _symbol: Symbol,
    ) -> Option<zed_extension_api::CodeLabel> {
        None
    }

    fn complete_slash_command_argument(
        &self,
        _command: zed_extension_api::SlashCommand,
        _args: Vec<String>,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String>
    {
        Ok(Vec::new())
    }

    fn run_slash_command(
        &self,
        _command: zed_extension_api::SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&zed_extension_api::Worktree>,
    ) -> zed_extension_api::Result<zed_extension_api::SlashCommandOutput, String> {
        Err("`run_slash_command` not implemented".to_string())
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &zed_extension_api::Project,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Err("`context_server_command` not implemented".to_string())
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &zed_extension_api::ContextServerId,
        _project: &zed_extension_api::Project,
    ) -> zed_extension_api::Result<Option<zed_extension_api::ContextServerConfiguration>> {
        Ok(None)
    }

    fn suggest_docs_packages(
        &self,
        _provider: String,
    ) -> zed_extension_api::Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    fn index_docs(
        &self,
        _provider: String,
        _package: String,
        _database: &zed_extension_api::KeyValueStore,
    ) -> zed_extension_api::Result<(), String> {
        Err("`index_docs` not implemented".to_string())
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: zed_extension_api::DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::DebugAdapterBinary, String> {
        Err("`get_dap_binary` not implemented".to_string())
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: zed_extension_api::serde_json::Value,
    ) -> zed_extension_api::Result<zed_extension_api::StartDebuggingRequestArgumentsRequest, String>
    {
        Err("`dap_request_kind` not implemented".to_string())
    }

    fn dap_config_to_scenario(
        &mut self,
        _config: zed_extension_api::DebugConfig,
    ) -> zed_extension_api::Result<zed_extension_api::DebugScenario, String> {
        Err("`dap_config_to_scenario` not implemented".to_string())
    }

    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        _build_task: zed_extension_api::TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<zed_extension_api::DebugScenario> {
        None
    }

    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        _build_task: zed_extension_api::TaskTemplate,
    ) -> zed_extension_api::Result<zed_extension_api::DebugRequest, String> {
        Err("`run_dap_locator` not implemented".to_string())
    }
    // ...
}

zed::register_extension!(MyExtension);
