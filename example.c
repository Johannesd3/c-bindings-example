#include "mylib.h"

int main(void) {
    mylib_some_struct *x = mylib_init();
    mylib_add(x, 1);
    mylib_add(x, 3);
    mylib_add(x, 5);
    mylib_print(x);
    mylib_add(x, 8);
    mylib_print(x);
    mylib_destroy(x);
    return 0;
}
