import { render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import type { Series } from '../../types';
import SeriesModal from '../../components/series/SeriesModal.vue';

describe('SeriesModal', () => {
  it('opens in create mode', () => {
    const { getByRole } = render(SeriesModal, {
      props: {
        modelValue: true,
        series: null,
      },
    });

    expect(
      getByRole('heading', { name: /create series/i }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Create' }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Cancel' }),
    ).toBeTruthy();
  });

  it('opens in edit mode', () => {
    const series: Series = {
      id: 1,
      title: 'Test',
      description: 'Test Description',
      publisher_id: '1',
      publisher: {
        id: '1',
        name: 'Test Publisher',
      },
      start_year: 2020,
      end_year: 2021,
      notes: '',
    };

    const { getByRole, getByDisplayValue } = render(SeriesModal, {
      props: {
        modelValue: true,
        series,
      },
    });

    expect(
      getByRole('heading', { name: /update series/i }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Update' }),
    ).toBeTruthy();

    expect(getByDisplayValue('Test')).toBeTruthy();
    expect(getByDisplayValue('Test Description')).toBeTruthy();
    expect(getByDisplayValue('Test Publisher')).toBeTruthy();
    expect(getByDisplayValue('2020')).toBeTruthy();
    expect(getByDisplayValue('2021')).toBeTruthy();
  });
});
