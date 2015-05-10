#!/usr/bin/env ruby

require_relative 'sections/root'

root_config = Sections::Root.new($stdin.read)
$stdout.print root_config.to_config
