#pragma once
#include <dlapi.h>
#include "result.h"

extern "C" struct SensorInfo {
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

extern "C" enum ReadoutMode {
  Low = 0,
  Medium = 1,
  High = 2,
  LowStackPro = 3,
  MediumStackPro = 4,
  HighStackPro = 5,
};

extern "C" struct ExposureInfo {
  float duration;
  bool light;
  enum ReadoutMode readout_mode;
  int bin_x;
  int bin_y;
};

extern "C" struct ExposeResult {
  unsigned short *buffer;
  size_t buffer_size;
};

void await(dl::IPromisePtr promise);

void free_gateway(dl::IGatewayPtr gateway);

dl::IGatewayPtr initialize_gateway();

Result<dl::ICameraPtr, int> initialize_camera(dl::IGatewayPtr gateway);
