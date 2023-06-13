#ifdef TEST

#include "unity.h"

#include "day1.h"

void setUp(void)
{
}

void tearDown(void)
{
}

void test_sum_calories(void)
{
    TEST_ASSERT_EQUAL_INT(24000, sum_calories("./src/day1/test"));
}

#endif // TEST
