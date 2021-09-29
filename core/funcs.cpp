#include <dlapi.h>
#include <fitsio.h>
#include <vector>
#include <cstring>
#include "utils.hpp"
#include "status.hpp"
#include "result.h"

Result<ExposeResult, const char *> expose(dl::ICameraPtr camera, dl::ISensorPtr sensor, ExposureInfo exp_info) {
  if (exp_info.bin_x != 1 || exp_info.bin_y != 1) {
    await(camera->queryCapability(dl::ICamera::eSupportsOnChipBinning));
    if (!camera->getCapability(dl::ICamera::eSupportsOnChipBinning)) {
      return Err("Binning requested, but camera does not support on-chip binning! Use --binx=1 and --biny=1, and perform binning yourself afterward.");
    }
    try {
      await(sensor->setSetting(dl::ISensor::UseOnChipBinning, 1));
    } catch (std::exception &ex) {
    return Err(ex.what());
    }
  } 

  auto sensor_info = get_sensor_info(sensor);

  dl::TSubframe subframe; 
  subframe.top = 0;
  subframe.left = 0;
  subframe.width = sensor_info.pixels_x;
  subframe.height = sensor_info.pixels_y;
  subframe.binX = 1;
  subframe.binY = 1;

  dl::TExposureOptions exposure_options;
  exposure_options.duration = std::max(exp_info.duration, sensor_info.exposure_duration_min);
  exposure_options.binX = exp_info.bin_x;
	exposure_options.binY = exp_info.bin_y;
	exposure_options.readoutMode = static_cast<int>(exp_info.readout_mode);
	exposure_options.isLightFrame = exp_info.light;
	exposure_options.useRBIPreflash = false;
	exposure_options.useExtTrigger = false;

	try {
		await(sensor->setSubframe(subframe));
	} catch (std::exception &ex) {
		return Err(ex.what());
	}

  // start exposure
  try {
    await(sensor->startExposure(exposure_options));
  } catch (std::exception &ex) {
    return Err(ex.what());
  }

  // wait for exposure to finish
  do {
    try {
      await(camera->queryStatus());	
    } catch (std::exception &ex) {
      return Err(ex.what());
    }
    auto status = camera->getStatus();
    if (status.mainSensorState == dl::ISensor::ReadyToDownload) break;
  } while (true);

  // get data
  try {
    await(sensor->startDownload());
  } catch (std::exception &ex) {
    return Err(ex.what());
  }

  auto image = sensor->getImage();

  /* auto buffer_length = image->getBufferLength(); */

  /* unsigned short *buffer; */
  /* buffer = (unsigned short *) malloc(sizeof(unsigned short) * buffer_length); */

  /* std::cout << "Moving to buffer" << std::endl; */
  /* std::memmove(buffer, image->getBufferData(), sizeof(unsigned short) * buffer_length); */
  /* std::cout << "Moved to buffer" << std::endl; */

  ExposeResult result;
  result.buffer = image->getBufferData();
  result.bufferlen = image->getBufferLength();
  result.metadata = image->getMetadata();
  result.expinfo = exposure_options;

  return Ok(result);
}

void save_image(ExposeResult expres, const char *filepath) {

  unsigned short * buffer = expres.buffer;
  unsigned int nelements = expres.bufferlen;
  auto expinfo = expres.expinfo;
  auto metadata = expres.metadata;

  fitsfile *fptr;
  int status = 0;
  long naxes[2] = { metadata.width, metadata.height };
  int bitpix = SHORT_IMG;
  const char *frametype = (expinfo.isLightFrame ? "Light Frame" : "Dark Frame");

  remove(filepath);

  fits_create_file(&fptr, filepath, &status);
  print_fits_err(status);
  fits_create_img(fptr, bitpix, 2, naxes, &status);
  print_fits_err(status);

  fits_write_date(fptr, &status);
  print_fits_err(status);
  fits_update_key(fptr, TFLOAT, "EXPOSURE", &metadata.exposureDuration, "Total exposure time in seconds", &status);
  print_fits_err(status);
  /* fits_update_key(fptr, TFLOAT, "EGAIN", &metadata.eGain, "Electronic gain in e-/ADU", &status); */
  /* print_fits_err(status); */
  fits_update_key(fptr, TUINT, "XBINNING", &metadata.binX, "Binning factor in width", &status);
  print_fits_err(status);
  fits_update_key(fptr, TUINT, "YBINNING", &metadata.binY, "Binning factor in height", &status);
  print_fits_err(status);
  fits_update_key_str(fptr, "IMAGETYP", frametype, "Type of image", &status);
  print_fits_err(status);

  fits_write_img(fptr, TSHORT, 1, nelements, buffer, &status);
  print_fits_err(status);
  fits_close_file(fptr, &status);
  print_fits_err(status);
}
