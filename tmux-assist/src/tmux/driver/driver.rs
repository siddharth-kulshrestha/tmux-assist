pub struct NewSessionRequest {
    name: String,
    format: String,
    width: int,
    height: int,
    detached: bool,
    env: [String],
    command: [String],
}

pub struct DisplayMessageRequest {
    pane: String,
    message: String,
}

pub struct CapturePaneRequest {
    pane: String,
    start_line: int,
    end_line: int,
}

pub trait Driver {
    /// runs the tmux new-session command and returns its output.
    pub fn new_session(&self, NewSessionRequest) -> Result<String, Err>;

    /// runs the tmux display-message command and returns its output.
    pub fn display_msg(&self, DisplayMessageRequest) -> Result<String, Err>;

    /// runs the tmux capture-pane command and returns its output.
    pub fn capture_pane(&self, CapturePaneRequest) -> Result<String, Err>;

    /// runs the tmux wait-for command, activating anyone waiting for this signal.
    pub fn send_signal(&self, String) -> Result<(), Err>;

    // runs the tmux wait-for command, waiting for a corresponding SendSignal command.
    pub fn wait_for_signal(&self, String) -> Result<(), Err>;

}

