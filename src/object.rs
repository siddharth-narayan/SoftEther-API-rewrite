// Lock object
struct LOCK
{
// 	void *pData;
// 	bool Ready;
// #ifdef	OS_UNIX
// 	UINT thread_id;
// 	UINT locked_count;
// #endif	// OS_UNIX
// #ifdef	_DEBUG
// 	char *FileName;
// 	UINT Line;
// 	UINT ThreadId;
// #endif	// _DEBUG
};

// Counter object
pub struct Counter {
	lock: *mut Lock,
	counter: u32,
	ready: bool,
}

// Reference counter
pub struct RefCounter
{
	counter: *mut Counter
}
