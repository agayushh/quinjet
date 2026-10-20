use super::TextBuffer;

pub(super) fn cycle_account(input: &mut TextBuffer, accounts: &[String], reverse: bool) {
    let current = accounts.iter().position(|login| *login == input.value);
    let next = current.map_or(0, |index| {
        if reverse {
            index
                .checked_sub(1)
                .unwrap_or_else(|| accounts.len().saturating_sub(1))
        } else {
            index
                .saturating_add(1)
                .checked_rem(accounts.len())
                .unwrap_or_default()
        }
    });
    if let Some(login) = accounts.get(next) {
        *input = TextBuffer::new(login.clone());
    }
}
