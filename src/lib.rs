
mod some_func;

use crate::some_func::some_func;

pub(crate) fn lib() {
    some_func();
}
