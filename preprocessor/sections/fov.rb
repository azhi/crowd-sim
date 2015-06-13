require_relative 'base'

module Sections
  class Fov < Base
    FOV_SECTION = 0x05
    FOV_ELEMENTS = {
      'forward' => 0x01, 'backward' => 0x02
    }

    field name: 'forward', type: :distribution, current_section: FOV_SECTION,
          element: FOV_ELEMENTS['forward']
    field name: 'backward', type: :distribution, current_section: FOV_SECTION,
          element: FOV_ELEMENTS['backward']
  end
end
