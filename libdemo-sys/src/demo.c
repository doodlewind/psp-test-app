#include <stdlib.h>

void my_print(int a);

void test_alloc() {
  void* p = malloc(80);
  p = realloc(p, 88);

  my_print((int)p);
}
