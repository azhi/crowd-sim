require_relative 'base'

module Sections
  class Forces < Base
    FORCES_SECTION = 0x04

    field name: 'target', type: :descendant, klass: 'Force::Target'
    field name: 'repulsion', type: :descendant, klass: 'Force::Repulsion'
  end
end

require_relative 'force/repulsion'
require_relative 'force/target'

