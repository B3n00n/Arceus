import { useState, useMemo } from 'react';
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
  Modal,
  Progress,
  App,
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
  useAllGameVersions,
  useUpdateGameVersion,
  useDeleteGameVersion,
} from '../hooks/useGameVersions';
import { useGames } from '../hooks/useGames';
import { GameVersionModal } from '../components/GameVersionModal';
import { api } from '../services/api';
import type { GameVersion } from '../types';

dayjs.extend(relativeTime);

const { Title } = Typography;

interface GameVersionWithGame extends GameVersion {
  game_name?: string;
}

export const GameVersionsPage = () => {
  const [modalOpen, setModalOpen] = useState(false);
  const [modalMode, setModalMode] = useState<'create' | 'edit'>('create');
  const [selectedVersion, setSelectedVersion] = useState<GameVersion | undefined>();
  const [searchText, setSearchText] = useState('');
  const [selectedGameFilter, setSelectedGameFilter] = useState<number | 'all'>('all');
  const [uploadProgress, setUploadProgress] = useState<number>(0);
  const [isUploading, setIsUploading] = useState(false);

  const { data: games = [] } = useGames();
  const gameIds = games.map(g => g.id);

  const { data: singleGameVersions = [], isLoading: isLoadingSingle, refetch: refetchSingle } =
    useGameVersions(typeof selectedGameFilter === 'number' ? selectedGameFilter : null);
  const { data: allVersions = [], isLoading: isLoadingAll, refetch: refetchAll } =
    useAllGameVersions(selectedGameFilter === 'all' ? gameIds : []);

  const versions = selectedGameFilter === 'all' ? allVersions : singleGameVersions;
  const isLoading = selectedGameFilter === 'all' ? isLoadingAll : isLoadingSingle;
  const refetch = selectedGameFilter === 'all' ? refetchAll : refetchSingle;

  const { message, modal } = App.useApp();
  const updateMutation = useUpdateGameVersion();
  const deleteMutation = useDeleteGameVersion();

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
        const { game_id, version, file } = values;

        if (!file) {
          message.error('Please select a ZIP file');
          return;
        }

        setIsUploading(true);
        setUploadProgress(0);
        setModalOpen(false);

        // Show progress modal
        const progressModal = modal.info({
          title: 'Uploading Game Version',
          content: (
            <div>
              <p>Uploading {file.name}...</p>
              <Progress percent={0} status="active" />
              <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
                This may take several minutes for large files. Do not close this window.
              </p>
            </div>
          ),
          okButtonProps: { style: { display: 'none' } },
          closable: false,
          maskClosable: false,
        });

        try {
          await api.uploadGameVersion(game_id, version, file, (progress) => {
            setUploadProgress(progress);
            progressModal.update({
              content: (
                <div>
                  <p>Uploading {file.name}...</p>
                  <Progress percent={progress} status="active" />
                  <p style={{ marginTop: 16, color: '#666', fontSize: 12 }}>
                    {progress < 100
                      ? 'This may take several minutes for large files. Do not close this window.'
                      : 'Processing... The ZIP will be extracted automatically.'}
                  </p>
                </div>
              ),
            });
          });

          progressModal.destroy();
          message.success(`Version ${version} uploaded successfully! Files will be extracted shortly.`);
          refetch();
          setSelectedVersion(undefined);
        } catch (error: any) {
          progressModal.destroy();
          const errorMessage = error.response?.data?.error || error.message || 'Upload failed';
          message.error(errorMessage);
        } finally {
          setIsUploading(false);
          setUploadProgress(0);
        }
      } else if (selectedVersion) {
        const { game_id, ...versionData } = values;
        await updateMutation.mutateAsync({
          gameId: selectedVersion.game_id,
          versionId: selectedVersion.id,
          data: versionData,
        });
        setModalOpen(false);
        setSelectedVersion(undefined);
      }
    } catch (error) {
      // Mutation error already handled by hook
    }
  };

  const handleModalCancel = () => {
    if (isUploading) {
      modal.confirm({
        title: 'Upload in progress',
        content: 'Are you sure you want to cancel? The upload will be interrupted.',
        onOk: () => {
          setModalOpen(false);
          setSelectedVersion(undefined);
          setIsUploading(false);
        },
      });
    } else {
      setModalOpen(false);
      setSelectedVersion(undefined);
    }
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
            options={[
              { label: 'All Games', value: 'all' },
              ...games.map((game) => ({
                label: game.name,
                value: game.id,
              })),
            ]}
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
        loading={isUploading || updateMutation.isPending}
      />
    </div>
  );
};
