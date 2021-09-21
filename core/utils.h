#pragma once
#include <dlapi.h>

struct SensorInfo {
  unsigned int pixels_x;
  unsigned int pixels_y;
  float pixel_size_x;
  float pixel_size_y;
  float cooler_setpoint_min;
  float cooler_setpoint_max;
  unsigned int bin_x_max;
  unsigned int bin_y_max;
  float exposure_duration_min;
  float exposure_precision;
};

enum ReadoutMode {
  Low = 0,
  Medium = 1,
  High = 2,
  LowStackPro = 3,
  MediumStackPro = 4,
  HighStackPro = 5,
};

struct ExposureInfo {
  float duration;
  bool light;
  ReadoutMode readout_mode;
  int bin_x;
  int bin_y;
};

void await(dl::IPromisePtr promise);

struct ExposeResult {
  unsigned short *buffer;
  size_t buffer_size;
};
