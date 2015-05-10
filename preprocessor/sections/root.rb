require_relative 'base'
require_relative 'forces'
require_relative 'scene'
require_relative 'spawn'
require_relative 'time'

module Sections
  class Root < Base
    def initialize(root_str)
      super(nil, root_str)
    end

    field name: 'scene', type: :descendant, klass: 'Scene'
    field name: 'time', type: :descendant, klass: 'Time'
    field name: 'spawn', type: :descendant, klass: 'Spawn'
    field name: 'forces', type: :descendant, klass: 'Forces'

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
  end
end
