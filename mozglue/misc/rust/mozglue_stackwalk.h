#ifndef mozglue_stackwalk_h
#define mozglue_stackwalk_h

extern "C" {
  void describe_code_address(void* image, void* pc);
}

#endif // mozglue_stackwalk_h
