require 'crack/xml'

require_relative 'base'

module Sections
  class Scene < Base
    SCENE_SECTION = 0x01
    SCENE_ELEMENTS = {
      'wall' => 0x01, 'spawn-area' => 0x02, 'target-area' => 0x03, 'panic-source' => 0x04,
      'width' => 0x11, 'height' => 0x12, 'scale' => 0x13,
      'file_name' => 0xFF
    }
    SCENE_ELEMENTS_TEMPLATES = {
      # x0 y0 x1 y1
      'wall' => 'S>S>S>S>',
      # x0 y0 x1 y1 id
      'spawn-area' => 'S>S>S>S>C',
      # x0 y0 x1 y1 id seq_no(7bit)|last(1bit)
      'target-area' => 'S>S>S>S>CC',
      # x y r power
      'panic-source' => 'S>S>S>C',

      'width' => 'S>', 'height' => 'S>',
      'scale' => 'E',
      'file_name' => 'S>A:len:'
    }

    field name: 'file', type: :custom, parser: :read_scene_file
    field name: 'scale', type: :float

    def to_config
      file_name = get_data('scene_file')
      file_name_template = SCENE_ELEMENTS_TEMPLATES['file_name'].sub(':len:', file_name.size.to_s)
      config = [SCENE_SECTION, SCENE_ELEMENTS['file_name'], file_name.size, file_name].pack(CONFIG_ITEM_TEMPLATE_PREFIX + file_name_template)

      config += get_data('geometry').inject("") do |geom_conf, (geom_el_type, geom_el_data)|
        data = [SCENE_SECTION, SCENE_ELEMENTS[geom_el_type], geom_el_data].flatten
        geom_conf + data.pack(CONFIG_ITEM_TEMPLATE_PREFIX + SCENE_ELEMENTS_TEMPLATES[geom_el_type])
      end
      config += [SCENE_SECTION, SCENE_ELEMENTS['scale'], get_data('scale')].pack(CONFIG_ITEM_TEMPLATE_PREFIX + SCENE_ELEMENTS_TEMPLATES['scale'])
      config += super.to_s
      config
    end

    def read_scene_file(file, &blc)
      data['scene_file'] = file

      scene_data = Crack::XML.parse(File.read(file))['svg']
      geometry = []
      geometry << ['width', scene_data['width'].to_i]
      geometry << ['height', scene_data['height'].to_i]

      Array[scene_data['circle']].select{ |circle| circle['x_csim_class'] == 'panic-source' }.each do |ps|
        geometry << [
          'panic-source',
          [ps['cx'], ps['cy'], ps['r'], ps['x_csim_power']].map(&:to_i)
        ]
      end

      scene_data['line'].select{ |line| line['x_csim_class'] == 'wall' }.each do |wall|
        geometry << [
          'wall',
          [wall['x1'], wall['y1'], wall['x2'], wall['y2']].map(&:to_i)
        ]
      end
      scene_data['rect'].select{ |rect| rect['x_csim_class'] == 'spawn-area' }.each do |spawn|
        geometry << [
          'spawn-area',
          [spawn['x'].to_i, spawn['y'].to_i,
           spawn['x'].to_i + spawn['width'].to_i, spawn['y'].to_i + spawn['height'].to_i,
           spawn['x_csim_id'].to_i]
        ]
      end
      scene_data['rect'].select{ |rect| rect['x_csim_class'] == 'target-area' }.each do |target|
        geometry << [
          'target-area',
          [target['x'].to_i, target['y'].to_i,
           target['x'].to_i + target['width'].to_i, target['y'].to_i + target['height'].to_i,
           target['x_csim_id'].to_i,
           (target['x_csim_seq_no'].to_i << 1) | (target['x_csim_last'] == 'true' ? 1 : 0)]
        ]
      end
      data['geometry'] = geometry
    end
  end
end
