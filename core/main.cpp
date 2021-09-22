#include "funcs.hpp"
#include "utils.hpp"
#include "status.hpp"
#include "CLI11.hpp"
#include <iostream>

int main(int argc, char** argv) {

  CLI::App app{"Dragonfly Narrowband core functions"};

  app.require_subcommand(1);

  // ---------------

  auto sub_cool = app.add_subcommand("cool", "Functions related to cooling and temperatures.");
  sub_cool->require_subcommand(1);

  sub_cool->add_subcommand("disable", "Turns off cooling.");
  sub_cool->add_subcommand("get", "Get the current temperatures for various parts of the system.");

  auto sub_cool_set = sub_cool->add_subcommand("set", "Enables cooling and sets the target cooling temperature.");

  float target_temp;
  sub_cool_set->add_option("temp", target_temp, "Target temperature in degrees C.")->required()->take_first();

  // ----------------

  auto sub_expose = app.add_subcommand("expose", "Take an exposure.");

  float duration;
  sub_expose->add_option("--duration", duration, "Duration of exposure in seconds.")->required();

  char *filepath;
  sub_expose->add_option("--file", filepath, "Location to save exposure to.")->required();

  bool dark;
  sub_expose->add_option("--dark", dark, "Take a dark instead of a light frame.");
  
  int bin_x = 1;
  sub_expose->add_option("--binx", bin_x, "Amount of binning for the x axis. Defaults to 1.");

  int bin_y = 1;
  sub_expose->add_option("--biny", bin_y, "Amount of binning for the y axis. Defaults to 1.");

  // ------------------
  
  CLI11_PARSE(app, argc, argv);
  
  // ------------------

	auto gateway = initialize_gateway();
  auto camera = initialize_camera(gateway).expect("Could not initialize camera!");

  if (sub_cool->parsed()) {
    auto cooler = initialize_cooler(camera).expect("Could not initialize cooler!");

    if (sub_cool->got_subcommand("disable")) {
      disable_cooler(cooler);
      std::cout << "Disabling cooler." << std::endl;
    } else if (sub_cool->got_subcommand("get")) {
      std::cout << get_temp_info(camera, cooler) << std::endl;
    } else if (sub_cool->got_subcommand("set")) {
      auto sensor = initialize_sensor(camera).expect("Could not initialize sensor!");
      float tgt = set_temp(cooler, sensor, target_temp);
      std::cout << "Setting temperature to " << tgt << " degrees C." << std::endl;
    }
  } 

  else if (sub_expose->parsed()) {
    auto sensor = initialize_sensor(camera).expect("Could not initialize sensor!");

    ExposureInfo expinfo;
    expinfo.bin_x = bin_x;
    expinfo.bin_y = bin_y;
    expinfo.duration = duration;
    expinfo.light = dark;
    expinfo.readout_mode = ReadoutMode::High;

    std::cout << "Exposure in progress..." << std::endl;
    dl::IImagePtr im = expose(camera, sensor, expinfo).unwrap();
    save_image(im, filepath);
    std::cout << "Image buffer saved to " << filepath << std::endl; 
  }

	free_gateway(gateway);
}
