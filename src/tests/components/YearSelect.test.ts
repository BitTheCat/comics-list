import { render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import YearSelect from '../../components/YearSelect.vue';

describe('YearSelect', () => {
  it('renders the correct range of years', () => {
    const { getByText } = render(YearSelect, {
      props: {
        minYear: 1950,
        maxYear: 2025,
      },
    });

    expect(getByText('1950')).toBeTruthy();
    expect(getByText('2025')).toBeTruthy();
  });
});
