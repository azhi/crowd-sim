type 'escape'

# scene description
scene do
  # svg file with scene geometry
  file '/home/azhi/develop/crowd-sim/resources/office/scene.svg'
  # file scale (meters per pixel)
  scale 0.1
end

# simulation time description
time do
  # time to end simulation (Float::INFINITY for infinite one), seconds
  end_time Float::INFINITY
  # simulation clock tick time
  tick 0.05
end

# spawn area description
spawn do
  # distribution of spawns in time
  time{ distribution 'uniform' }
  # rate of spawns (men in second)
  rate 1.0
end

# forces description
forces do
  # force that pushes man to his target
  target do
    # speed distribution
    speed{ distribution 'normal'; mean 1.5; std_deviation 0.2 }
  end
  # force that pushes man apart from each other
  repulsion do
    # force coeff distribution
    coeff{ distribution 'normal'; mean 1.0; std_deviation 0.01 }
  end
end

# field of view description
fov do
  forward{ distribution 'normal'; mean 5.0; std_deviation 0.1 }
  backward{ distribution 'normal'; mean 0.3; std_deviation 0.001 }
end

# density map description
density_map do
  enabled false
  min_threshold 4.0
  max_threshold 10.0
end
