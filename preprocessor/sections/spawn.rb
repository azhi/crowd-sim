require_relative 'base'

module Sections
  class Spawn < Base
    SPAWN_SECTION = 0x03
    SPAWN_ELEMENTS = {
      'rate' => 0x01,
      'time' => 0x02
    }
    SPAWN_ELEMENTS_TEMPLATES = {
      'rate' => 'E'
    }

    field name: 'rate', type: :float
    field name: 'time', type: :distribution, current_section: SPAWN_SECTION,
          element: SPAWN_ELEMENTS['time'],
          context_defaults: {'from' => 0.0, 'to' => 'ref:time.end_time'}

    def to_config
      config = ""
      config += [SPAWN_SECTION, SPAWN_ELEMENTS['rate'], get_data('rate')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + SPAWN_ELEMENTS_TEMPLATES['rate'])
      config += super.to_s
      config
    end
  end
end
