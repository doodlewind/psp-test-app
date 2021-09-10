#include <stdlib.h>

void my_print(int a);

void test_alloc() {
  void* p = malloc(1000);

  int a = (int)realloc(p, 20);
  my_print(a);

  int b = (int)realloc(p, 88);
  my_print(b);
}
