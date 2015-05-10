require_relative '../base'

module Sections::Force
  class Repulsion < Sections::Base
    REPULSION_SUBSECTION = 0x0100
    REPULSION_COEFF_ELEMENT = REPULSION_SUBSECTION | 0x01

    field name: 'coeff', type: :distribution, current_section: Sections::Forces::FORCES_SECTION,
          element: REPULSION_COEFF_ELEMENT
  end
end
