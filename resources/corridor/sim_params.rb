# scene description
scene do
  # svg file with scene geometry
  file '/home/azhi/develop/crowd-sim/resources/corridor/scene.svg'
  # file scale (meters per pixel)
  scale 0.05
end

# simulation time description
time do
  # time to end simulation (Float::INFINITY for infinite one), seconds
  end_time 20.0
  # simulation clock tick time
  tick 0.1
end

# spawn area description
spawn do
  # distribution of spawns in time
  time{ distribution 'uniform' }
  # rate of spawns (men in second)
  rate 3
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
    coeff{ distribution 'normal'; mean 10.3; std_deviation 0.7 }
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
  min_threshold 7.0
  max_threshold 15.0
end
