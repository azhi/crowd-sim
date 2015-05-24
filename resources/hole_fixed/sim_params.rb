# scene description
scene do
  # svg file with scene geometry
  file '/home/azhi/develop/crowd-sim/resources/hole_fixed/scene.svg'
  # file scale (meters per pixel)
  scale 0.05
end

# simulation time description
time do
  # time to end simulation (Float::INFINITY for infinite one), seconds
  end_time 120.0
  # simulation clock tick time
  tick 0.05
end

# spawn area description
spawn do
  # distribution of spawns in time
  time{ distribution 'uniform' }
  # rate of spawns (men in second)
  rate 0.8
end

# forces description
forces do
  # force that pushes man to his target
  target do
    # speed distribution
    speed{ distribution 'normal'; mean 1.5; std_deviation 0.3 }
  end
  # force that pushes man apart from each other
  repulsion do
    # force coeff distribution
    coeff{ distribution 'normal'; mean 1.0; std_deviation 0.1 }
  end
end

# field of view description
fov do
  forward 5.0
  backward 0.1
end

# density map description
density_map do
  enabled true
  min_threshold 5.0
  max_threshold 10.0
end
