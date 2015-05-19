require_relative 'base'

module Sections
  class Fov < Base
    FOV_SECTION = 0x05
    FOV_ELEMENTS = {
      'forward' => 0x01, 'backward' => 0x02
    }

    FOV_ELEMENTS_TEMPLATES = {
      'forward' => 'E', 'backward' => 'E'
    }

    field name: 'forward', type: :float
    field name: 'backward', type: :float

    def to_config
      config = ""
      config += [FOV_SECTION, FOV_ELEMENTS['forward'], get_data('forward')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + FOV_ELEMENTS_TEMPLATES['forward'])
      config += [FOV_SECTION, FOV_ELEMENTS['backward'], get_data('backward')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + FOV_ELEMENTS_TEMPLATES['backward'])
      config += super.to_s
      config
    end
  end
end
