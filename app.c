#include "rusttest.h"

#include <stdio.h>

int main()
{
	printf("a + b = %u\n", rusty_add(3, 5));
	return 0;
}
