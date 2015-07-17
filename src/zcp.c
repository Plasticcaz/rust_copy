/*
 * ~zcp~
 * by Plasticcaz
 *
 * This is my implementation of a copy program in C.
 */

#include <stdio.h>

#define BUFFER_SIZE 100000

int main(int argc, char **argv)
{
  if (argc == 3)
  {
    FILE *input = fopen(argv[1], "rb");

    if (input)
    {
      char buffer[BUFFER_SIZE];
      const int num_bytes = fread(buffer, sizeof(char), BUFFER_SIZE, input);
      fclose(input);

      if (num_bytes > BUFFER_SIZE)
      {
        printf("ERROR: File is too big.\n");;
        return -1;
      }

      FILE *output = fopen(argv[2], "wb");

      if (output)
      {
          fwrite(buffer, sizeof(char), num_bytes, output);
          fclose(output);
      }
    }
    else
    {
      printf("Error opening: %s\n", argv[1]);
    }
  }
  else
  {
    printf("FORMAT: %s %s %s\n", argv[0], "<source_file>", "<destination>");
  }
  return 0;
}
