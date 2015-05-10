require_relative '../base'

module Sections::Force
  class Target < Sections::Base
    TARGET_SUBSECTION = 0x0200
    TARGET_SPEED_ELEMENT = TARGET_SUBSECTION | 0x01

    field name: 'speed', type: :distribution, current_section: Sections::Forces::FORCES_SECTION,
          element: TARGET_SPEED_ELEMENT
  end
end
