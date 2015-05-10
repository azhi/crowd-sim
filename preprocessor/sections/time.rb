require_relative 'base'

module Sections
  class Time < Base
    TIME_SECTION = 0x02
    TIME_ELEMENTS = {
      'end_time' => 0x01, 'tick' => 0x02
    }
    TIME_ELEMENTS_TEMPLATES = {
      # 32 bit time, FFFFFFFF - infinite
      'end_time' => 'L>',
      'tick' => 'E'
    }

    field name: 'end_time', type: :float
    field name: 'tick', type: :float

    def to_config
      config = ""
      end_time = get_data('end_time')
      end_time_value = end_time == Float::INFINITY ? 2 ** 32 - 1 : end_time.round
      config += [TIME_SECTION, TIME_ELEMENTS['end_time'], end_time_value].pack(CONFIG_ITEM_TEMPLATE_PREFIX + TIME_ELEMENTS_TEMPLATES['end_time'])
      config += [TIME_SECTION, TIME_ELEMENTS['tick'], get_data('tick')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + TIME_ELEMENTS_TEMPLATES['tick'])
      config += super.to_s
      config
    end
  end
end
