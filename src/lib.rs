pub trait UnwrapUnchecked {
    /// The type of the inner value.
    type Output;

    /// Moves the inner value out of `self`.
    /// 
    /// # Panics
    /// 
    /// In debug builds, this function panics if the inner value does not exist.
    /// 
    /// # Safety
    /// 
    /// In optimized builds, this function does not check if the inner value exists.
    unsafe fn unwrap_unchecked(self) -> Self::Output;
}

impl<T> UnwrapUnchecked for Option<T> {
    type Output = T;

    #[inline]
    unsafe fn unwrap_unchecked(self) -> Self::Output {
        if let Some(value) = self {
            return value;
        }
        unreachable()
    }
}

impl<T, E> UnwrapUnchecked for Result<T, E> {
    type Output = T;

    #[inline]
    unsafe fn unwrap_unchecked(self) -> Self::Output {
        if let Ok(value) = self {
            return value;
        }
        unreachable()
    }
}

#[cfg(debug_assertions)]
#[inline(always)]
unsafe fn unreachable() -> ! {
    unreachable!()
}

#[cfg(not(debug_assertions))]
#[inline(always)]
unsafe fn unreachable() -> ! {
    std::hint::unreachable_unchecked()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_some_unwrap_unchecked() {
        assert!(unsafe { Some(()).unwrap_unchecked() } == ());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_option_none_unwrap_unchecked() {
        unsafe { Option::<()>::None.unwrap_unchecked() };
    }

    #[test]
    fn test_result_ok_unwrap_unchecked() {
        assert!(unsafe { Result::<_, ()>::Ok(()).unwrap_unchecked() } == ());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_result_error_unwrap_unchecked() {
        assert!(unsafe { Result::<(), _>::Err(()).unwrap_unchecked() } == ());
    }
}