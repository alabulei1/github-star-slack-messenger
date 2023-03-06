use github_flows::{listen_to_event, EventPayload};
use slack_flows::send_message_to_channel;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let owner: &str = "WasmEdge";
    let repo: &str = "WasmEdge";

    listen_to_event(owner, repo, vec!["star"], |payload| handler(repo, payload)).await;

    Ok(())
}

async fn handler(repo: &str, payload: EventPayload) {
    let slack_workspace_name: &str = "secondstate";
    let slack_channel_name: &str = "github-status";

    if let EventPayload::UnknownEvent(e) = payload {
        let stargazers_count = e["repository"]["stargazers_count"].as_i64().unwrap_or(-1);

        let text =
            format!("Congratulations on your repository {repo} with {stargazers_count} stars.");

        if stargazers_count % 10 == 0 {
            send_message_to_channel(slack_workspace_name, slack_channel_name, text);
        }
    }
}
