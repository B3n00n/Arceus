import { Select } from 'antd';
import { useGames } from '../hooks/useGames';

interface GameSelectorProps {
  value?: number[];
  onChange?: (value: number[]) => void;
  disabled?: boolean;
}

export const GameSelector = ({ value, onChange, disabled }: GameSelectorProps) => {
  const { data: games = [], isLoading } = useGames();

  const options = games.map((game) => ({
    label: game.name,
    value: game.id,
  }));

  return (
    <Select
      mode="multiple"
      value={value}
      onChange={onChange}
      options={options}
      disabled={disabled}
      placeholder="Select games this arcade can access"
      loading={isLoading}
      style={{ width: '100%' }}
      allowClear
      showSearch
      filterOption={(input, option) =>
        (option?.label?.toString() || '').toLowerCase().includes(input.toLowerCase())
      }
    />
  );
};
