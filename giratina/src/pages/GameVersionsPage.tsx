import { useState, useMemo, useEffect } from 'react';
import {
  Typography,
  Button,
  Table,
  Space,
  Input,
  Card,
  Popconfirm,
  Tooltip,
  Flex,
  Select,
  Tag,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  SearchOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import {
  useGameVersions,
  useCreateGameVersion,
  useUpdateGameVersion,
  useDeleteGameVersion,
} from '../hooks/useGameVersions';
import { useGames } from '../hooks/useGames';
import { GameVersionModal } from '../components/GameVersionModal';
import type { GameVersion } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;

// Extended type to include game name
interface GameVersionWithGame extends GameVersion {
  game_name?: string;
}

export const GameVersionsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedVersion, setSelectedVersion] = useState<GameVersion | undefined>();
  const [searchText, setSearchText] = useState('');
  const [selectedGameFilter, setSelectedGameFilter] = useState<number | undefined>();

  const { data: games = [] } = useGames();
  const { data: versions = [], isLoading, refetch } = useGameVersions(selectedGameFilter || null);
  const createMutation = useCreateGameVersion();
  const updateMutation = useUpdateGameVersion();
  const deleteMutation = useDeleteGameVersion();

  // Enrich versions with game names
  const enrichedVersions: GameVersionWithGame[] = useMemo(() => {
    return versions.map((version) => ({
      ...version,
      game_name: games.find((g) => g.id === version.game_id)?.name || 'Unknown',
    }));
  }, [versions, games]);

  const filteredVersions = useMemo(() => {
    return enrichedVersions.filter((version) =>
      version.version.toLowerCase().includes(searchText.toLowerCase()) ||
      version.gcs_path.toLowerCase().includes(searchText.toLowerCase()) ||
      version.game_name?.toLowerCase().includes(searchText.toLowerCase())
    );
  }, [enrichedVersions, searchText]);

  // Auto-select first game if none selected
  useEffect(() => {
    if (games.length > 0 && !selectedGameFilter) {
      setSelectedGameFilter(games[0].id);
    }
  }, [games, selectedGameFilter]);

  const handleCreate = () => {
    setModalMode('create');
    setSelectedVersion(undefined);
    setModalOpen(true);
  };

  const handleEdit = (version: GameVersion) => {
    setModalMode('edit');
    setSelectedVersion(version);
    setModalOpen(true);
  };

  const handleDelete = async (version: GameVersion) => {
    await deleteMutation.mutateAsync({
      gameId: version.game_id,
      versionId: version.id,
    });
  };

  const handleModalSubmit = async (values: any) => {
    try {
      if (modalMode === 'create') {
        const { game_id, ...versionData } = values;
        await createMutation.mutateAsync({
          gameId: game_id,
          data: versionData,
        });
      } else if (selectedVersion) {
        const { game_id, ...versionData } = values;
        await updateMutation.mutateAsync({
          gameId: selectedVersion.game_id,
          versionId: selectedVersion.id,
          data: versionData,
        });
      }
      setModalOpen(false);
      setSelectedVersion(undefined);
    } catch (error) {
      // Error handling is done in the mutation hooks
    }
  };

  const handleModalCancel = () => {
    setModalOpen(false);
    setSelectedVersion(undefined);
  };

  const columns: ColumnsType<GameVersionWithGame> = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      width: 80,
      sorter: (a, b) => a.id - b.id,
    },
    {
      title: 'Game',
      dataIndex: 'game_name',
      key: 'game_name',
      width: 200,
      render: (name: string) => <Tag color="purple">{name}</Tag>,
      sorter: (a, b) => (a.game_name || '').localeCompare(b.game_name || ''),
    },
    {
      title: 'Version',
      dataIndex: 'version',
      key: 'version',
      width: 120,
      sorter: (a, b) => a.version.localeCompare(b.version),
      render: (version: string) => <strong>{version}</strong>,
    },
    {
      title: 'GCS Path',
      dataIndex: 'gcs_path',
      key: 'gcs_path',
      render: (path: string) => <code style={{ fontSize: 12 }}>{path}</code>,
    },
    {
      title: 'Release Date',
      dataIndex: 'release_date',
      key: 'release_date',
      width: 180,
      render: (date: string) => (
        <Tooltip title={dayjs(date).format('YYYY-MM-DD HH:mm:ss')}>
          {dayjs(date).fromNow()}
        </Tooltip>
      ),
      sorter: (a, b) => dayjs(a.release_date).unix() - dayjs(b.release_date).unix(),
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 120,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Tooltip title="Edit">
            <Button
              type="text"
              icon={<EditOutlined />}
              onClick={() => handleEdit(record)}
              size="small"
            />
          </Tooltip>
          <Popconfirm
            title="Delete version?"
            description="This will also remove assignments using this version."
            onConfirm={() => handleDelete(record)}
            okText="Delete"
            okButtonProps={{ danger: true }}
            cancelText="Cancel"
          >
            <Tooltip title="Delete">
              <Button
                type="text"
                danger
                icon={<DeleteOutlined />}
                size="small"
              />
            </Tooltip>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <Flex justify="space-between" align="center" style={{ marginBottom: 24 }}>
        <Title level={2} style={{ margin: 0 }}>
          Game Versions
        </Title>
        <Button
          type="primary"
          icon={<PlusOutlined />}
          onClick={handleCreate}
          size="large"
        >
          Create Version
        </Button>
      </Flex>

      <Card>
        <Flex gap="middle" style={{ marginBottom: 16 }} wrap="wrap">
          <Select
            placeholder="Filter by game"
            style={{ width: 200 }}
            value={selectedGameFilter}
            onChange={setSelectedGameFilter}
            options={games.map((game) => ({
              label: game.name,
              value: game.id,
            }))}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
          <Input
            placeholder="Search versions..."
            prefix={<SearchOutlined />}
            allowClear
            style={{ width: 300 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
          />
          <Tooltip title="Refresh">
            <Button
              icon={<ReloadOutlined />}
              onClick={() => refetch()}
              loading={isLoading}
            />
          </Tooltip>
        </Flex>

        <Table
          columns={columns}
          dataSource={filteredVersions}
          loading={isLoading}
          rowKey="id"
          pagination={{
            pageSize: 10,
            showTotal: (total) => `Total ${total} versions`,
            showSizeChanger: true,
            pageSizeOptions: ['10', '20', '50', '100'],
          }}
        />
      </Card>

      <GameVersionModal
        open={modalOpen}
        mode={modalMode}
        version={selectedVersion}
        onSubmit={handleModalSubmit}
        onCancel={handleModalCancel}
        loading={createMutation.isPending || updateMutation.isPending}
      />
    </div>
  );
};
