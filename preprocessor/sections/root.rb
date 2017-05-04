require_relative 'base'
require_relative 'forces'
require_relative 'scene'
require_relative 'spawn'
require_relative 'time'
require_relative 'fov'
require_relative 'density_map'

module Sections
  class Root < Base
    def initialize(root_str)
      super(nil, root_str)
    end

    GENERAL_SECTION = 0x00
    GENERAL_ELEMENTS = {'type' => 0x01}
    GENERAL_ELEMENTS_TEMPLATES = {'type' => 'C'}

    field name: 'type', type: :enum, values: {'flow' => 0x01, 'escape' => 0x02}, default: 'flow'
    field name: 'scene', type: :descendant, klass: 'Scene'
    field name: 'time', type: :descendant, klass: 'Time'
    field name: 'spawn', type: :descendant, klass: 'Spawn'
    field name: 'forces', type: :descendant, klass: 'Forces'
    field name: 'fov', type: :descendant, klass: 'Fov'
    field name: 'density_map', type: :descendant, klass: 'DensityMap'

    def get_ref_value(value)
      value = value.sub('ref:', '')
      path = value.split(?.)
      value = path.inject(data) do |res, key|
        next unless res
        res = if key.start_with?(?[) && key.end_with?(?])
          res.find{ |(ar_el_type, _ar_el_value)| ar_el_type == key[1...-1] }.last
        else
          res[key]
        end
        res = res.data if Base === res
        res
      end
      value
    end

    def to_config
      config = ""
      config += [GENERAL_SECTION, GENERAL_ELEMENTS['type'], get_data('type')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + GENERAL_ELEMENTS_TEMPLATES['type'])
      config += super.to_s
      config
    end
  end
end
