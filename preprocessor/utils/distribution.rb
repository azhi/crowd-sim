require_relative '../sections/base'

module Utils
  class Distribution < ::Sections::Base
    attr_reader :context_defaults, :current_section, :element

    DISTRIBUTION_TYPES = {
      'uniform' => 0x01, 'normal' => 0x02,
      'time_infinite' => 0x03
    }

    DISTRIBUTION_TEMPLATES = {
      # type from to
      'uniform' => 'CEE',
      # type mean std_deviation
      'normal' => 'CEE',
      # type avg_rate rate_deviation
      'time_infinite' => 'CEE'
    }

    field name: 'distribution', type: :string

    field name: 'from', type: :float
    field name: 'to', type: :float

    field name: 'mean', type: :float
    field name: 'std_deviation', type: :float

    field name: 'avg_rate', type: :float
    field name: 'rate_deviation', type: :float

    def fields
      self.class.fields.map do |field_name, field|
        field = field.merge(default: context_defaults[field[:name]]) unless field[:default]
        [field_name, field]
      end.to_h
    end

    def initialize(parent, context_defaults, current_section, element, &blc)
      @context_defaults, @current_section, @element = context_defaults, current_section, element
      super(parent, &blc)
    end

    def to_config
      config = ""
      distribution = get_data('distribution')
      raise ArgumentError, "Unknow distribution: #{distribution}" unless %w[uniform normal time_infinite].include?(distribution)
      distribution_template = DISTRIBUTION_TEMPLATES[distribution]
      case distribution
      when 'uniform'
        config += [current_section, element, DISTRIBUTION_TYPES[distribution], get_data('from'), get_data('to')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + distribution_template)
      when 'normal'
        config += [current_section, element, DISTRIBUTION_TYPES[distribution], get_data('mean'), get_data('std_deviation')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + distribution_template)
      when 'time_infinite'
        config += [current_section, element, DISTRIBUTION_TYPES[distribution], get_data('avg_rate'), get_data('rate_deviation')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + distribution_template)
      end
      config += super.to_s
      config
    end

    def config_param(field)
      [current_section, distribution_elements[field], get_data(field)].pack(CONFIG_ITEM_TEMPLATE_PREFIX + DISTRIBUTION_ELEMENTS_TEMPLATES[field])
    end
  end
end
