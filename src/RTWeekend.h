/**
 * @file RTWeekend.h
 * @author your name (you@domain.com)
 * @brief
 * Generally utility header file to include for main.cpp
 * @version 0.1
 * @date 2023-10-14
 *
 * @copyright Copyright (c) 2023
 *
 */
#ifndef RTWEEKEND_H
#define RTWEEKEND_H

// Including all our libs
#include "Camera.h"
#include "Color.h"
#include "Constants.h"
#include "Hittable.h"
#include "Hittable_List.h"
#include "Interval.h"
#include "Objects.h"
#include "Ray.h"
#include "Vec3.h"

// Libs needed also included
#include <cmath>
#include <limits>
#include <memory>

// Utility Functions
double degrees_to_radians(double degrees);

color::Color color_ray(const ray::Ray &r,
                       const hittable_list::Hittable_List &world);

#endif
