import json
from datetime import datetime
from typing import Callable, Dict, List, Optional, Union
from abc import ABC, abstractmethod
from feast import type_map
from feast.data_source import DataSource
from feast.protos.feast.core.DataSource_pb2 import DataSource as DataSourceProto
from feast.repo_config import RepoConfig
from feast.value_type import ValueType
import pyarrow


class YummyDataSource(DataSource):
    """Custom data source class for local files"""

    def __init__(
        self,
        name: Optional[str] = "",
        event_timestamp_column: Optional[str] = "",
        created_timestamp_column: Optional[str] = "",
        field_mapping: Optional[Dict[str, str]] = None,
        date_partition_column: Optional[str] = "",

    ):
        super().__init__(
            event_timestamp_column=event_timestamp_column,
            created_timestamp_column=created_timestamp_column,
            field_mapping=field_mapping,
            date_partition_column=date_partition_column,
            description: Optional[str] = "",
            tags: Optional[Dict[str, str]] = None,
            owner: Optional[str] = "",
        )

    @abstractmethod
    @property
    def reader_type(self):
        """
        Returns the reader type which will read data source
        """
        ...

    @staticmethod
    def from_proto(data_source: DataSourceProto):
        """
        Creates a `CustomFileDataSource` object from a DataSource proto, by
        parsing the CustomSourceOptions which is encoded as a binary json string.
        """
        raise NotImplementedError

    def to_proto(self) -> DataSourceProto:
        """
        Creates a DataSource proto representation of this object, by serializing some
        custom options into the custom_options field as a binary encoded json string.
        """
        raise NotImplementedError

    @staticmethod
    @abstractmethod
    def source_datatype_to_feast_value_type() -> Callable[[str], ValueType]:
        """
        Returns the callable method that returns Feast type given the raw column type.
        """
        raise NotImplementedError

    def get_table_column_names_and_types(
        self, config: RepoConfig
    ) -> Iterable[Tuple[str, str]]:
        """
        Returns the list of column names and raw column types.
        Args:
            config: Configuration object used to configure a feature store.
        """
        raise NotImplementedError

    def get_table_query_string(self) -> str:
        """
        Returns a string that can directly be used to reference this table in SQL.
        """
        raise NotImplementedError

class YummyDataSourceReader(ABC):

    @abstractmethod
    def read_datasource(
        self,
        data_source,
        features: List[str],
        backend_type: BackendType,
        entity_df: Optional[Union[pd.DataFrame, Any]] = None,
    ) -> Union[pyarrow.Table, pd.DataFrame, Any]:
        ...


