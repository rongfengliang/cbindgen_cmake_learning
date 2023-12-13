#include <stdio.h>
#include "bindings.h"
int main(){
   int result =  addv2(13,33);
   printf("Result: %d\n", result);
   Foo foo = {
      .a = 1,
      .b = 2,
   };
   bool init_result = init_app(foo);
   printf("init_result: %d\n", init_result);
}