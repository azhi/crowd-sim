require_relative 'base'

module Sections
  class DensityMap < Base
    DM_SECTION = 0x06
    DM_ELEMENTS = {
      'enabled' => 0x01, 'min_threshold' => 0x02, 'max_threshold' => 0x03
    }

    DM_ELEMENTS_TEMPLATES = {
      'enabled' => 'C', 'min_threshold' => 'E', 'max_threshold' => 'E'
    }

    field name: 'enabled', type: :bool
    field name: 'min_threshold', type: :float
    field name: 'max_threshold', type: :float

    def to_config
      config = ""
      config += [DM_SECTION, DM_ELEMENTS['enabled'], get_data('enabled') ? 1 : 0].pack(CONFIG_ITEM_TEMPLATE_PREFIX + DM_ELEMENTS_TEMPLATES['enabled'])
      config += [DM_SECTION, DM_ELEMENTS['min_threshold'], get_data('min_threshold')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + DM_ELEMENTS_TEMPLATES['min_threshold'])
      config += [DM_SECTION, DM_ELEMENTS['max_threshold'], get_data('max_threshold')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + DM_ELEMENTS_TEMPLATES['max_threshold'])
      config += super.to_s
      config
    end
  end
end
