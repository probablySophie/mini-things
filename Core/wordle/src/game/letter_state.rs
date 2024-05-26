
#[derive(Clone, Copy, Default)]
pub enum LetterState
{
	Perfect,
	Almost,
	Wrong,
	#[default] Unused,
}
