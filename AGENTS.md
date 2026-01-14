 ***IMPORTANT: when retrieving content from urls, use the fetch tools built-in to the agent environment, DO NOT USE `curl` or a python script or some other crazy solution! DO NOT USE CURL!!!*** (Exceptions: OpenAI Codex, Github Copilot _CLOUD_ session *MAY* use curl. ***VSCODE SESSIONS MAY NEVER USE CURL!***)

Get a list files of all the files in the agent guidelines repo from the url: `https://api.github.com/repos/bcolloran/agents_md/git/trees/main?recursive=1`. 


Refer to the file main file `AGENTS.md` in the repo `https://github.com/bcolloran/agents_md` for general guidelines, and to the other files as needed for specific instructions. ***Err on the side of reading all the files in the repo if you are unsure which apply.***

This repo must be accessed by the agent at the start of every task to ensure it has the latest coding standards and guidelines.

**If the agent is not able to access that repo for any reason (network issues, offline mode, etc.), it must immediately halt work on the task and report the issue to the user.**
