module Sections
  class Base
    attr_reader :parent, :data

    CONFIG_ITEM_TEMPLATE_PREFIX = "CS>"

    def self.field(name:, type: nil, klass: nil, parser: nil, default: nil,
                   current_section: nil, element: nil, context_defaults: {})
      @fields ||= {}
      @fields[name] = {name: name, type: type, klass: klass, parser: parser, default: default,
                       current_section: current_section, element: element,
                       context_defaults: context_defaults}
      define_method name do |val = nil, &blc|
        field = self.class.fields[name]
        data[name] = get_value(field, val, &blc)
      end
    end

    def self.fields
      @fields
    end

    def fields
      self.class.fields
    end

    def initialize(parent, str = nil, &blc)
      @parent = parent
      @data = {}
      if str
        instance_eval(str)
      elsif blc
        instance_eval(&blc)
      else
        raise ArgumentError, 'No string or block to evaluate provided'
      end
    end

    def get_data(key)
      return data[key] if data[key]
      field = fields[key]
      data[key] = get_value(field, nil) if field
    end

    def get_value(field, value, &blc)
      value ||= field[:default]
      value = get_ref_value(value) if String === value && value.start_with?('ref:')
      value = case field[:type]
              when :int then value.to_i
              when :float then value.to_f
              when :string then value.to_s
              when :descendant
                klass = field[:klass].split('::').inject(Sections) do |res, kl|
                  raise 'Requested descendant class not found!' unless res
                  res.const_get(kl)
                end
                klass.new(self, &blc)
              when :distribution
                ::Utils::Distribution.new(self, field[:context_defaults],
                                          field[:current_section], field[:element],
                                          &blc)
              when :custom
                case field[:parser]
                when Symbol then self.send(field[:parser], value, &blc)
                when Proc then field[:parser].(value, &blc)
                end
              end
      value
    end

    def get_ref_value(value)
      parent.get_ref_value(value)
    end

    def to_config
      descendants_to_config
    end

    private

      def descendants_to_config
        fields.select{ |_field_name, field| %i[descendant distribution].include? field[:type] }.inject('') do |res, (_field_name, field)|
          descendant = data[field[:name]]
          res + descendant.to_config
        end
      end
  end
end

require_relative '../utils/distribution'
