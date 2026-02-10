use std::env;

pub fn resolve_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_editor_uses_env_var() {
        // SAFETY: test is run single-threaded via --test-threads=1
        unsafe { env::set_var("EDITOR", "nvim") };
        assert_eq!(resolve_editor(), "nvim");
        unsafe { env::remove_var("EDITOR") };
    }

    #[test]
    fn resolve_editor_falls_back_to_vim() {
        // SAFETY: test is run single-threaded via --test-threads=1
        unsafe { env::remove_var("EDITOR") };
        assert_eq!(resolve_editor(), "vim");
    }
}
